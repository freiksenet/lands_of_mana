# Real time 4X game inspired by Master of Magic, Dominions and Paradox GSGs

## Random refactorings

- [ ] Render should add transforms to world etc
- [ ] Camera should be child of world or player
- [ ] Layer code doesn't need layer ids in lookups
- [ ] Move out camera transform from interact code
- [ ] Make Selected a component again
- [ ] Make animation type a component that changes, instead of a reaction ot selection
- [ ] Borders and UI need to be connected to game world for deconstructing

## Project plan

- [x] Phase 1 - get familiar with rust/bevy
  - [x] tilemap rendering
  - [x] unit rendering
  - [x] ui interactions
- [ ] Phase 2 - basic game play prototype - building blocks to develop gameplay
  - [ ] game tick and days
    - [ ] ui showing tick and days
  - [ ] resources
  - [ ] ui top bar
  - [ ] concept of entities using and giving resources
  - [ ] selection ui
    - [ ] screen for selected entity
    - [ ] city
    - [ ] units
    - [ ] sites
  - [ ] moving and ordering units
    - [ ] basic pathfinding
    - [ ] terrain, terrain movement cost, blocking terrain
      - [ ] roads
      - [ ] rivers
      - [ ] cliffs
      - [ ] forests
      - [ ] mountains
  - [ ] city and city buildings
  - [ ] sites and site control
  - [ ] building new units
  - [ ] unit stats
  - [ ] building new cities
  - [ ] unexplored terrain, fog of war, visibility
  - [ ] spellcasting ui and spending mana to cast spell
  - [ ] prototype "big window" ui for things like research, agent, etc
  - [ ] notification and event ui
  - [ ] prototype spells
    - [ ] persistent spell with upkeep
    - [ ] time limited spell
    - [ ] instant spell
    - [ ] spell to change unit stats
    - [ ] spell to change resource production
    - [ ] spell to damage unit
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