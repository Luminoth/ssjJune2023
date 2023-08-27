#! /usr/bin/env python3

# TODO: this probably could live in Terraform

import boto3

client = boto3.client('dynamodb')

# client config
print('creating client config')
client.put_item(TableName='ssj2023', Item={
    'type': {'S': 'config'},
    'id': {'S': 'client'},
    'max_characters': {'N': '3'},
    'max_character_name_len': {'N': '16'},
})

print('done!')
