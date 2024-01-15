#!/bin/sh

NAME=$1
mv $NAME $NAME.gz

name=$NAME.gz
gzip -d "$NAME.gz"

mv $NAME $NAME.idk
