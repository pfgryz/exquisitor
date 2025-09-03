
## TODO
- posprawdzać czy się nie zmieniły numery sekcji
- posprawdzać czy się nie zmieniły strony

## TODO
[[Rev 4, issue 6]] [[Rev 2, issue 4]] - lista ze zmianami
[[Rev 3, issue 7]] [[Rev 3, issue 1]] [[Rev 2, issue 5b]] - podziękować

# Statuses: PFG
```dataview 
TABLE 
	length(rows.file.name) as numfiles , 
	join(rows.file.link, ", ") as files 
flatten 
	file.tags as tag 
WHERE 
   (contains(tag, "#todo")
   OR contains(tag, "#ongoing")
   OR contains(tag, "#extend")  
   OR contains(tag, "#review") 
   OR contains(tag, "#toPush")
   OR contains(tag, "#done")
   OR contains(tag, "#wontFix"))
   AND owner = "PFG"
GROUP BY tag 
```

# Statuses: RN
```dataview 
TABLE 
	length(rows.file.name) as numfiles , 
	join(rows.file.link, ", ") as files 
flatten 
	file.tags as tag 
WHERE 
   (contains(tag, "#todo")
   OR contains(tag, "#ongoing")
   OR contains(tag, "#extend")  
   OR contains(tag, "#review") 
   OR contains(tag, "#toPush")
   OR contains(tag, "#done")
   OR contains(tag, "#wontFix"))
   AND owner = "RN"
GROUP BY tag 
```
