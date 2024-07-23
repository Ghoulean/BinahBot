Put BaseMod in root directoy of this package (so it is on the same level as `./build` and `./.cargo`)

# How to BaseMod:

1. Install Library of Ruina on Steam (Windows)
2. Install BaseMod Nightly from Steam Workshop. Do **not** confuse this with regular BaseMod as regular BaseMod has known issues.
3. Open Library of Ruina, launch with mods, and enable BaseMod
4. Load your save file in your preferred locale, then quit back to menu
5. Repeat step 4 with every locale
6. BaseMod folder should be generated in mods directory in steam folder: `/<Steam installation root folder>/steamapps/common/Library Of Ruina/LibraryOfRuina_Data/Mods/`

# Assumptions about the XML that's only encoded in the game logic

There are a few differences between raw XML and the results of this serialization (that I know of/remember):

1. Due to limitations with the XML library, a missing element cannot be differentiated with an empty element (e.g. `<Desc></Desc>`, `</Desc>`, and `(empty)` are not differentiated among each other)
   1. Battle symbol localization has some entries with no `Desc` element and others with `""` as the text value of the `Desc` element. Due to limitations with the XML library these two cannot be differentiated, so all `""` `Desc` elements are treated as missing.
2. Battle symbol missing `NoAppear` defaults to `false`
3. Keywords dynamically added to certain types of combat pages (such as `Instance_Keyword` for onplay pages) not included in this reparser
4. Missing die type (`Detail`) defaults to "slash"
5. Key pages have the following defaults:
   1. Missing resistances default to `Normal` on all
   2. Missing starting light and starting max light defaults to `3/3`
   3. Missing base number of dice defaults to `1`
   4. Missing range defaults to `Melee`

Reparser encodes these additional properties into its generated game objects.