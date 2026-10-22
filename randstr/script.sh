# Script to add a seizure to my clipboard /s /j

EXECUTABLE=./target/debug/randstrclipboards

case $1 in
    "generate")
        $EXECUTABLE generate 
        ;;
    "start")
        while true; do
            $EXECUTABLE copyclip &> /dev/null
            sleep 2
            $EXECUTABLE resetclip &> /dev/null
        done
        ;;
    "help")
        ./target/debug/randstrclipboards help
        ;;
    *)
        echo "Unknown Command: $1. Check Docs and try again!"
        ;;
esac