#!/bin/bash

result=`curl -X POST -H "Content-Type: application/json" -d '{"params": "{query}"}' http://localhost:12004/search`

array=`echo $result | jq -c '.[]'`

echo '<?xml version="1.0"?><items>'
for item in "${array[@]}"; do
	title=`echo $item | jq -r '.title'`
	url=`echo $item | jq -r '.url'`
	echo "<item arg=\"$url\"><title>$title</title><icon>icon.png</icon></item>"
done

echo '</items>'
