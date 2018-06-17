
# Quickstart guide

This file contains the description of command line instructions and the expected result.

## How to use

All instructions require a working configuration to interact with kvasiruence. For configuration you can click here: [setup](127.0.0.1/example)


##### Open Wiki-Page read only

`` kvasir -ro "page" ``

##### Edit an wiki-page 

`` kvasir -e "page" ``

##### Adding new page from file

`` kvasir -a file "page" ``

##### Adding new empty page
`` kvasir -a --empty "page" ``

##### Deleting a wiki page
`` kvasir -d "page" ``

##### Creating default config

creating an example config with placeholders 

`` kvasir -c /path/to/config ``

##### Change path to configuration file

`` kvasir -cp /path/to/config ``

##### Searching in Wiki

You can search for pages that contains an certain string like that:

`` kvasir -s "SEARCH_STRING" ``

##### Working with page ids

instead of the page title you can also use the page id for every command like this:

`` kvasir -e -id 1234 ``
