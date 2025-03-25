# database structure

- header [100 bytes reserved]

- collections:

## sleipnir-master [collection that information about tables]

first entry after header is 'sleipnir-master' (entry to itself)

[id: 0, content: [next_document, collection_name: 'sleipnir-master'], next_collection: (offset_to_next_entry)]


second entry is 'sleipnir-reusable' (document that contains information about this collection, e.g.
first entry offset)
[id: 1, content: [next_document, collection_name: 'sleipnir-reusable'], next_collection: (offset_to_next_entry)]