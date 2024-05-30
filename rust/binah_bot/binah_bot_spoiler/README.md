# binah_bot_spoiler

Simple spoiler enforcement configuration builder package. **Note that at the moment this only affects combat pages and key pages only** until I continue work with the reparser. Note that channels with a spoiler configuration will also figure out combat+key pages with an unspecified chapter.

Create a file with the name `channel_config.toml` and put it in the same level as `binah_bot_spoiler/src/lib.rs`. I'll think up a better place to put it later.

Key: channel ID. Value: Chapter enums as string. Chapter enums of `None` are ignored.

Example usage:

```toml
1234567890123456789=StarOfTheCity
1234567890123456788=Canard
1234567890123456787=ImpuritasCivitatis
```