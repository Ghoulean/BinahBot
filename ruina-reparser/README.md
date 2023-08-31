# Ruina Reparser

Hack scripts to convert Library of Ruina XML files into an easily digestable format. ~~These scripts are one-off. Don't expect good code here.~~

EDIT: I expected this to be 1-off but considering how important data parsing is, I think working on this package is actually a long-term thing now. So eventually I will need to completely refactor this, since my current adhoc abstractions are showing its weaknesses.

First: set up Node 18. Not tested with other versions of node. You can use Node version manager.

1. Download XMLs from BaseMod
2. Put BaseMod folder into root
3. Build `@ghoulean/ruina-common` (should be in the same repo)
4. Build this package via `npm run build`
