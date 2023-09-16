you can use partial .html files in your _theme with the syntax

{{ partial "partial.html" . }}

which will include partial.html

additionally, you may pass arguments as a comma separated list

{{ partial "backlink.html" . (index, Home Page, modified date) }}

which will then replace {{0}}, {{1}}, {{2}} in the partial.html file with the arguments
"index", "Home Page", "modified date" respectively