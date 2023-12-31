# Cobalt Localization

### Useful information
* Localizations are stored in JSON files called ``table.json`` which contain key-value pairs for the translations. All the key-value pairs currently in use are in [``message/us/usen/table.json``](https://github.com/DivineDragonFanClub/localize/blob/master/message/us/usen/table.json), which you can refer to.
* We use the same regional names as ``fe_assets_message`` for familiarity, so if the language you want to work on is missing, feel free to create the appropriate directories.
* It is totally fine to not have every key-value in a language's ``table.json`` file. If you do not know the proper terminology for one of the translations, please leave out that key-value; we'd rather you not contribute lines you are unsure about!
* You only need to worry about the [``message``](https://github.com/DivineDragonFanClub/localize/tree/master/message) directory, as the ``src`` one contains the code we use in Cobalt to load the translations.

As a last note, feel free to add yourself to the "authors" field so you can be credited appropriately in due time!

### Localization style
The language of reference for Cobalt is US English, but we are aiming for something closer to a localization than a literal translation.

This means we'd rather you translate and stylize the text in a way that fits with the official localization in the respective language, capitalization and all, even if it deviates slightly from the presentation found in the US English text.

### Example

For example, if you are translating to European French, we'd rather you go with the European French way of doing things even if it does not aesthetically match with the American English text displayed by Cobalt.

American English
![Minimap Rotation](https://cdn.discordapp.com/attachments/345670285040680982/1134853994628518000/image.png)

European French
![Rotation mini-carte](https://cdn.discordapp.com/attachments/345670285040680982/1134854202447900733/image.png)

## How to contribute

First, you will need a Github account to contribute, so make sure this is taken care of!

Once you do, navigate to the file of your choice (or use the ``Add file`` dropdown in the top right if it is missing) and start editing! Github will automatically offer to create a "fork" and create a "pull request" with your change.  
If you are sure that you are done, confirm and write a nice title and description in your pull request so that we can review it and make sure everything is fine.

That's it! Real simple, right?
