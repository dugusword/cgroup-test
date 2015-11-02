#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )" 
cgexec -g memory:biye $DIR/cgroup-test/target/release/cgroup-test
