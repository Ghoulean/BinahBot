Put BaseMod in root directoy of this package (so it is on the same level as `./build` and `./.cargo`)

There are a few differences between raw XML and the results of this serialization (that I know of/remember):
1. Due to limitations with the XML library, a missing element cannot be differentiated with an empty element (e.g. `<Desc></Desc>`, `</Desc>`, and `(empty)` are not differentiated among each other)
2. Abno of `AbnoPage` is explicitly mentioned
3. Combat page `ScriptDesc`s omitted
4. Die `Desc`s omitted
5. Missing die type (`Detail`) defaults to "slash"
6. Keywords automatically added to certain types of combat pages (such as `Instance_Keyword` for onplay pages) not included in this reparser
7. Key pages have added defaults:
   1. Missing resistances default to `Normal` on all
   2. Missing starting light and starting max light defaults to `3/3`
   3. Missing base number of dice defaults to `1`
   4. Missing range defaults to `Melee`
8. Key page `TextId` XML element ignored. `ID` attribute is taken as the "true" ID.
9. Passive `Level`s omitted
10. Passive `NeedLevel` omitted
11. Passive `CanReceivePassive` omitted
12. Battle symbol missing `NoAppear` defaults to `false`
13. Battle symbol localization has some entries with no `Desc` element and others with `""` as the text value of the `Desc` element. Due to limitations with the XML library these two cannot be differentiated, so all `""` `Desc` elements are treated as missing.

While I did these without thinking at first, I believe I may later reconsider this decision of whether to reparse by XML only, if I should encode game assumptions also, or some "reasonable" combination of both approaches.

## TODO: This package works, but there's a ton of copy paste and crap. Refactor until it's right

# How to BaseMod:

1. Install Library of Ruina on Steam (Windows)
2. Install BaseMod Nightly from Steam Workshop. Do **not** confuse this with regular BaseMod as regular BaseMod has known issues.
3. Open Library of Ruina, launch with mods, and enable BaseMod
4. Load your save file in your preferred locale, then quit back to menu
5. Repeat step 4 with every locale
6. BaseMod folder should be generated in mods directory in steam folder: <todo: get folder path>
