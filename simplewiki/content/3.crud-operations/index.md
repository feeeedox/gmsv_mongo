---
title: CRUD Operations
description: Choose a method to insert, query, update, or delete documents
navigation:
  title: CRUD Operations
  order: 1
---

# CRUD Operations

Work with documents through a collection obtained with `db:Collection(name)`. Each method page includes synchronous calls and async callback examples.

## Method Reference

| Task | Synchronous method | Async method |
| --- | --- | --- |
| Insert | [`InsertOne`](/crud-operations/insert#insertone) | [`InsertOneAsync`](/crud-operations/insert#insertoneasync) |
| Insert | [`InsertMany`](/crud-operations/insert#insertmany) | [`InsertManyAsync`](/crud-operations/insert#insertmanyasync) |
| Query | [`Find`](/crud-operations/query#find) | [`FindAsync`](/crud-operations/query#findasync) |
| Query | [`FindOne`](/crud-operations/query#findone) | [`FindOneAsync`](/crud-operations/query#findoneasync) |
| Query | [`Count`](/crud-operations/query#count) | [`CountAsync`](/crud-operations/query#countasync) |
| Update | [`UpdateOne`](/crud-operations/update#updateone) | [`UpdateOneAsync`](/crud-operations/update#updateoneasync) |
| Update | [`UpdateMany`](/crud-operations/update#updatemany) | [`UpdateManyAsync`](/crud-operations/update#updatemanyasync) |
| Delete | [`DeleteOne`](/crud-operations/delete#deleteone) | [`DeleteOneAsync`](/crud-operations/delete#deleteoneasync) |
| Delete | [`DeleteMany`](/crud-operations/delete#deletemany) | [`DeleteManyAsync`](/crud-operations/delete#deletemanyasync) |

::link-card{to="/operators" title="Operators" description="Find filters and update operators in one place."}
::

::link-card{to="/crud-operations/async" title="Async Operations" description="Understand callbacks and nonblocking calls."}
::
