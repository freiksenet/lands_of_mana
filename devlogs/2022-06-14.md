# (DRAFT) Devlog 2022-06-15 - Selecting things

​I have been working on how selecting something in the game works. There is a lot of interactions in real-time strategies - you select units, buildings, special actions. Some units can be drag selected. Many actions are contextual to what is currently selected. This is an interesting challenge, especially when implementing it in a ECS based engine like Bevy.

I started with an idea that all entities that can be selected will be marked with `Selectable` component. As some things are selectable by dragging (eg units), while others are only selectable one-by-one (eg buildings), I also added `SelectableMany` component to additionally mark those. As it's relevant to a current user to know who is selected, I've added a `Selection` enum component to a current player (`Viewer` component) entity. `Selection` can be either `None`, `Unit` or `Units`. In latter variants it takes either an `Entity` or a `Vec<Entity>`.

In initial design, I decided to add a `Selected` and `Selecting` components to entities that have been selected or are hovering. I put them to `SparseSet` storage in Bevy ECS, so that adding and removing them is cheap. This was a decent solution for displaying units as rendered - I could walk through `Added` and `Removed` components in ui sync stage and add or update selection box. However when I started thinking about more complex cases it started being more complicated. For example, there is always a lot of context when interacting. Having something selected means that hovering might be contextual, eg to show that you can target units or capture building. This would lead to a complex menagerie of checking and moving around state component tags. As components are added only during syncronization, this would require even more different stages (or a frame skip) and it started to feel like a wrong approach.

In addition to the above issue, there is actually two kinds of views you need to have on currently selected context. From one point of view, being selected (or hovered / interacted in other way), is a inherent trait of certain entity like a unit or tile. They need to be able to display a selection box or a different targetting cursor. On the other hand, having something selected is something you need to know as a set of all items you have selected from a point of view of GUI or performing actions on entities. For example you need to have a window showing your selected units and issuing a command should send the to all units selected. Therefore we have a slight conflict of which information is actual source of truth.

After realizing the latter point, I've decided that the whole current selection is an inherent property of current player, rather that the world or unit entities. Selection has no actual purpose beyond the UI - you don't have any special world changes that affect only selected units, it is just an abstraction to making control easier. I have a marker `Viewer` component that indicates player entity that is currently playing the game at current computer. I've added components to indicate various UI states to it. I've added `Selection` component, that contains an enum that indicates the kind of selection viewer currently has. In my case it's `None`, `Unit(Entity)` and `Units(HashSet<Entity>)`, but in future it will be extended to include stuff like cities or spells that you have selected to cast. Then I've added a `CursorTarget` component, that indicates what entity is currently being targetted and what kind of entity it is. I've implemented some methods on Selection component that allowed me to implement UI logic like figuring out if unit is selected or adding unit to a selection in a nice encaplusated manner.

```rust
#[derive(Component, Debug, Clone, Default)]
pub struct Selection(SelectionType);

#[derive(Debug, Clone, Default)]
pub enum SelectionType {
    #[default]
    None,
    Unit(Entity),
    Units(HashSet<Entity>),
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        matches!(self.0, SelectionType::None)
    }

    pub fn is_selected(&self, entity: Entity) -> bool {
        match &self.0 {
            SelectionType::Unit(selected_entity) if *selected_entity == entity => true,
            SelectionType::Units(selected_entities) if selected_entities.contains(&entity) => true,
            _ => false,
        }
    }

    // ... more

    pub fn clear(&mut self) {
        self.0 = SelectionType::None;
    }

    pub fn select_unit(&mut self, entity: Entity) {
        self.0 = SelectionType::Unit(entity);
    }

    // add unit to a valid unit selection, ignores otherwise
    pub fn add_unit_to_selection(&mut self, entity: Entity) {
        match &self.0 {
            SelectionType::Units(selected_entities) => {
                let mut new_selected_entities = selected_entities.clone();
                new_selected_entities.insert(entity);
                self.0 = SelectionType::Units(new_selected_entities);
            }
            SelectionType::Unit(selected_entity) if *selected_entity != entity => {
                let mut selected_entities = HashSet::new();
                selected_entities.insert(*selected_entity);
                selected_entities.insert(entity);
                self.0 = SelectionType::Units(selected_entities);
            }
            SelectionType::None => {
                self.0 = SelectionType::Unit(entity);
            }
            _ => {}
        }
    }

   // ... more
}
```

Next thing was to add rendering of the selection box. Each sprite needs to be a separate component, and as selection box can be of different size, I've decided to use a separate component per corner. In addition I've added a parent component to those and added it as both a child of `Unit` entity (and later other `Selectable` entities), but also via a direct link with `WithSelectionBox` component. This way I can retrieve `WithSelectionBox` component and then get the Box entity without iterating through all unit children (for example individual unit figures or some other future ui element). In addition to entity `WithSelectionBox` also contains an enum of possible kinds of selection box.

All selectable entities get `WithSelectionBox` with a `SelectionBoxDisplayType::None`. I use change trackers to see if `Selection` or `CursorSelectionTarget` changes and then update the entity's `WithSelectionBox` component's display type with a correct type. I do it during `Sync` label in my `UiSync` stage. After that in a `Update` label I update corner's sprite sheet index and make them visible if display type isn't None.

This all works quite well and supports future work for drag selection and for contextual UI based on selected entities. One optimization here would be to have a better data structure to map `Position` to all entities that have it. This way we won't need to iterate over all entities with `Position` to set such contextual UI states.

Next steps are implementing drag selection box and Kayak UI elements to display units.

TODO - add info about selection box