#!/bin/bash

# Copy config file to /etc
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
sudo cp $PWD/cgconfig.conf /etc/cgconfig.conf 
sudo cgconfigparser -l /etc/cgconfig.conf
