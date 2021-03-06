# Lands of Mana (working title) - Real time 4X game inspired by Master of Magic, Dominions and Paradox GSGs

## Running

- `cargo run` runs the game.
- Assets and maps aren't licensed with Apache License, but are proprietary. Submodule in git has them, you need access to that module for assets.
- You can run `cargo run --bin style_guide` to see GUI style guide (widgets available that are ready)

### Assets and maps

Assets and maps aren't licensed with Apache License, but are proprietary. Submodule in git has them, you need access to that module for assets.

## Game engine description

Game uses Bevy, a ECS engine written in Rust. Tis pretty cool. Also uses the following libraries of note:

1. `iyes_loopless` - better game states and fixed timestep stages, based on Bevy's RFC
2. `bevy_ecs_tilemap` - nice tilemap rendering. We use `rewrite` branch.
3. `leafwing-input-manager` - input and keybindings. Nice abstractions for action indirection.
4. `bevy_kira_audio` - audio stuff, haven't used it, looks good
5. `bevy_asset_loader` - nice asset loading
6. `bevy_egui` - immediate mode gui

We render stuff in 2d, trying to do it pixel art style (mostly not very pixel perfect).

### Rust Modules

- `prelude` - export all top levels and common deps (bevy, loopless) so that you can just import that
- `assets` - all asset loading stuff
- `config` - states, stages and labels
- `game` - game world entities and systems, reacting to game actions, loading map, game tick
- `gui` - egui stuff
- `render` - all rendering related stuff
- `ui` - user input stuff and input abstractions (selected). should prolly rename to `input` or `control` or smth.

### Folders

- `assets` - submodule with assets from a closed repo

### Entity structure

### Stages, labels and ordering

`config.rs` has fancy stuff to order labels and after stuff automatically, so you call `label_and_after` to set a label and put systems to after.

Normal frame operates in two stages - bevy's default Update and UiSync. Update reacts to input and issues world actions (label `Input`) and then updates game world based on world actions (`GameActions` label). In UiSync stage, GUI bindings are updated and changes are made to components that indicate what needs to be rendered based on game world (`Sync` label). `Update` label does majority of graphic changes (so changes to Spritesheets, Transforms etc should happen there).

`EngineState` is used for sequencing loading, but probably is overly complicated for no reason. Lots of loading graphics can probably happen dynamically based on entities that don't have corresponding compononts for rendering. After `EngineState` reaches the `InGame`, most systems start running (other states mostly have enter/exit systems only). In future I'd guess `MainMenu` would be a state and then `LoadingGame` state that might have substates if they require ordering (or just labels).

When game is unpaused (`InGameState::Running`), every fixed timestep (currently just 1s, should be controllable) `GameTick` stage happens. `Tick` label does actual increment, then `UpdateEntities` should see if new things have spawned or old things should despawn (like if movement finished, combat round happened, if a unit died, if a unit finished building) and then `UpdateResources` does upkeep. Upkeep and income is done for _future_ tick, so added things do it, but removed things won't.

### Tidbits and various random observations

- x 0, y 0 is bottom-left corner
  - Multi-tile entities (city) positions are always bottom-left

## Random short term todos

- [ ] Render should add transforms to world etc
- [ ] Refactor structs that are single value to just be single value
- [ ] Default values for some sentinels, group into bundlesn better
- [ ] Make separate module for selection rendering stuff
- [ ] Make separate module for selection input handling stuff
- [ ] Remove need of using patched egui
- [ ] World initialization for relevant parts should happen in relevant plugins, not in game (eg ViewerBundle in ui)
- [ ] ViewerMap should respect ordering and output stable ordering
- [ ] Mouse controls for map
- [ ] Province borders re-enable / redesign
- [ ] There shoulrd be a logic to sorting selection target
- [ ] Unit figure sprite rotation for movement
- [ ] Revisit selection methods
- [ ] Remake selection to indicate what is selected - multiple cities don't make sense
- [ ] Direction indicators for units
- [ ] Sites not as a tilemap object
- [ ] Refactor the tile selection stuff
- [ ] Camera transform thingie is broken when moving camera
- [ ] Badges
  - [ ] Unit badge for visible units only
  - [ ] Figure out unit badge selection and ordering (and if it should also offset when unit moves - yes likely)
  - [x] Make unit animation for moving order actually apply to unit transform

## Project plan

- [x] Phase 1 - get familiar with rust/bevy
  - [x] tilemap rendering
  - [x] unit rendering
  - [x] ui interactions
- [ ] Phase 2 - basic game play prototype - building blocks to develop gameplay
  - [x] game tick and days
    - [x] ui showing tick and days
  - [x] resources
  - [x] ui top bar
  - [x] concept of entities using and giving resources
  - [ ] selection ui
    - [x] selection by clicking and selection box
    - [x] drag selection
    - [x] screen for selected entity
    - [ ] city
    - [ ] units
    - [ ] sites
  - [ ] moving and ordering units
    - [x] basic pathfinding
    - [ ] terrain, terrain movement cost, blocking terrain
      - [ ] roads
      - [ ] rivers
      - [ ] cliffs
      - [ ] forests
      - [ ] mountains
    - [ ] unit speeds
    - [ ] actual pathfinding
  - [ ] city and city buildings
  - [ ] building new units
  - [ ] unit stats
  - [ ] unit combat
  - [ ] building new cities
  - [ ] spellcasting ui, mages etc
  - [ ] prototype "big window" ui for things like research, agent, etc
  - [ ] prototype spells
    - [ ] persistent spell with upkeep
    - [ ] time limited spell
    - [ ] instant spell
    - [ ] spell to change unit stats
    - [ ] spell to change resource production
    - [ ] spell to damage unit
  - [ ] notification and event ui
  - [ ] unexplored terrain, fog of war, visibility
  - [ ] contextual cursors and click effects
    - [x] display cursor image
    - [ ] cursor click effect
    - [ ] cursor types based on context
- [ ] Phase 3 - exploring prototype gameplay - implementing all systems without content
  - [ ] combat, protected sites
  - [ ] basic neutral mob ai
  - [ ] sites spawning raiders
  - [ ] sites gameplay
    - [ ] how are they found
    - [ ] how are they captured
    - [ ] sites as anomalies
  - [ ] agents game play
  - [ ] spell research gameplay / ui
  - [ ] auxillary menu/ui
    - [ ] start game, save game, load game
    - [ ] in game menu
  - [ ] basic victory point win condition mechanic
  - [ ] decent looking premade map to explore all map possibilities
- [ ] Phase 4 - content to prove gameplay loop in solitaire mode
  - [ ] moving data into serializable editable way
  - [ ] possibly some kind of editor for eg map
  - [ ] developing "final" content for limited set of options
  - [ ] two fantasy species
    - [ ] enough spells to fill a randomized game book
    - [ ] enough sites for diverse game play
    - [ ] either world Gen or pre-made map
  - [ ] single player no enemy mage gameplay with win condition
  - [ ] graphic, ui etc basic polish
  - [ ] music, sounds, spell effects
- [ ] Phase 5 - content diversity - iterating over gameplay loop, vs mode etc
  - [ ] map gen
  - [ ] enough varied content
  - [ ] figuring out vs ai gameplay or mp gameplay (which is easier)
  - [ ] evaluating content needs irt graphics, music etc
