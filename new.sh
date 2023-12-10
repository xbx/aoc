#!/bin/zsh

set -x
set -e
LAST=$(ls | grep day- | tail -n1 | tr -d "\n")

LAST_NUMBER=$(echo $LAST | egrep -o -E "[0-9]*")

NEW_NUMBER=$(( $LAST_NUMBER + 1 ))

if [ $NEW_NUMBER -lt 10 ]; then
    NEW_NUMBER=0$NEW_NUMBER
fi

cp -r $LAST day-$NEW_NUMBER

cd day-$NEW_NUMBER

rm $(ls src/bin/*rs | tail -n1)

mv $(ls src/bin/*rs | tail -n1) src/bin/d$NEW_NUMBER-1.rs

sed -i '' "s/day_$LAST_NUMBER/day_$NEW_NUMBER/" src/bin/d$NEW_NUMBER-1.rs
sed -i '' "s/day-$LAST_NUMBER/day-$NEW_NUMBER/" Cargo.toml

cp src/bin/d$NEW_NUMBER-1.rs src/bin/d$NEW_NUMBER-2.rs
sed -i '' "s/part1/part2/" src/bin/d$NEW_NUMBER-2.rs
