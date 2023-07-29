# Cobalt's localization

### Useful informations
* You only need to worry about the ``message`` directory, as the ``src`` one contains the code we use in Cobalt to load the translations.
* We use the same regional names as ``fe_assets_message`` for familiarity, so if the language you want to work on is missing, feel free to create the appropriate directories.
* Localizations are stored in a JSON files called ``table.json`` which contain nothing but pairs of key-value for the translations. All the key-value currently in use are in ``message/us/usen/table.json``, which you can refer to.
* Assuming you do not know the proper terminology for one of the translations, it is totally fine to not have every key-value in the file. We'd rather you do not contribute lines you are unsure about!

As a last note, feel free to add yourself to the "authors" field so you can be credited appropriately in due time!

### Localization style
The language of reference for Cobalt is US English, but we are aiming for something closer to a localization than a literal translation.

This means we'd rather you translate and stylize the text in a way that fits with the official localization in the respective language, capitalization and all, even if it deviates slightly from the intended way in the US English text.

### Example

American English
![Minimap Rotation](https://cdn.discordapp.com/attachments/345670285040680982/1134853994628518000/image.png)

European French
![Rotation mini-carte](https://cdn.discordapp.com/attachments/345670285040680982/1134854202447900733/image.png)

Assuming you translate to European French, we'd rather you go with the European French way of doing things even if it does not aesthetically match with the American English text displayed by Cobalt.

## How to contribute

First, you will need a Github account to contribute, so make sure this is taken care of!

Once you do, navigate to the file of your choice (or use the ``Add file`` dropdown in the top right if it is missing) and start editing! Github will automatically offer to create a "fork" and "create a pull request" with your change.  
If you are sure that you are done, confirm and write a nice title and description in your pull request so that we can review it and make sure everything is fine.

That's it! Real simple, right?
