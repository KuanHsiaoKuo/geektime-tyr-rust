fswatch -x --event Updated */*.md | while read file
do
    event_info=($file)
    echo "${event_info[0]} was updated"
    vim -c 'Ir' ${event_info[0]} < /dev/tty
done
