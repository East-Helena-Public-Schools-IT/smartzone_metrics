#!/bin/bash
cat error.json | cut -c -$(cat error_column) | awk -F, '{print $NF}'
