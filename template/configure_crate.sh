#!/usr/bin/env bash
#
# initialize a new rust project based on this template
#
# USAGE: TODO


CURRENT_DIR=$(basename $(pwd -P))
if [[ $CURRENT_DIR == "_CRATENAME_" ]]; then
	STARTING_AT_TEMPLATE=1
fi

CRATENAME=$1

if [[ $CRATENAME != "" ]]; then

	if [[ $STARTING_AT_TEMPLATE ]]; then
		if [[ -e $(readlink -f "../$CRATENAME") ]]; then
			echo "I'm sorry. The name '$CRATENAME' already exists in the parent folder."
			exit 1
		else
			echo -e "\nWARNING: the template will be copied as '$CRATENAME'\n"
		fi
	else
		echo -e "\nWARNING: the current template will be initialized as '$CRATENAME'\n"
	fi
else
	echo -e "usage: ./configure_crate.sh name"
	exit 2
fi

#
prompt="do you really want to do it? y/n? "
while true; do
	read -sp "$prompt" -n1 res
	if [[ $res == 'y' || $res == 'Y' ]]; then
		echo -e "\n[y] Ok let's go!\n"
		LIBERA_CONFIRM_CRATE_CONFIGURATION=true
		break
	# elif [[ $res == 'n' || $res == 'N]' ]]; then
	# NOTE: anything that is not yes, is a no.
	else
		echo "[n] Ok bye!"
		exit 3
	# else
	# 	echo -n "."
	# 	prompt=""
	fi
done

# CONTENT_SAFEGUARDS (WIP)
res=0
ask=$(grep "_CRATENAME_" Cargo.toml)
if [[ $? -ne 0 ]]; then ((res+=1)); fi
if [[ -z $ask ]]; then ((res+=1)); fi

ask=$(grep -r "_CRATENAME_" .git/)
if [[ $? -ne 0 ]]; then ((res+=1)); fi
if [[ -z $ask ]]; then ((res+=1)); fi

if [[ res -ne 0 ]]; then
	echo "I'm sorry, this template has already been modifed. Aborting..."
	exit 4
fi

# MODIFICATIONS

if [[ $STARTING_AT_TEMPLATE ]]; then
	echo "- copying the _CRATENAME_ directory to '../$CRATENAME'..."
	cd ..
	cp -r "_CRATENAME_" "$CRATENAME"
	cd "$CRATENAME"
fi

if [[ $LIBERA_CONFIRM_CRATE_CONFIGURATION ]]; then
	echo "- renaming in Cargo.toml..."
	sed -i "s/_CRATENAME_/$CRATENAME/g" Cargo.toml

	echo "- renaming in README.md..."
	sed -i "s/_CRATENAME_/$CRATENAME/g" README.md

	echo "- renaming _info/info-.md..."
	mv "_info/info-.md" "_info/info-$CRATENAME.md"
	sed -i "s/_CRATENAME_/$CRATENAME/g" "_info/info-$CRATENAME.md"

	echo "- deleting '.git' and creating a new one..."
	rm -rf .git
	git init --quiet

	echo "- updating '.gitignore'..."
	sed -i '/_info/d' .gitignore

	echo "- deleting 'Cargo.lock'..."
	rm -rf Cargo.lock

	echo "- deleting 'configure_crate.sh'..."
	rm configure_crate.sh
fi

echo -e "\nDone!"
if [[ $STARTING_AT_TEMPLATE ]]; then
	echo -e "You can go to '../$CRATENAME/ now."
fi
