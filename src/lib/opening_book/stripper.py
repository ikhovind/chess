import subprocess
import re
import os

bashCommand = "sort oneline.pgn -o oneline.pgn"
blankrm= "grep . oneline.pgn > opening.pgn"

with open("out.pgn") as w, open("oneline.pgn", "w") as nw:
    for line in w:
        if line[0] != "[":
            if line.strip():
                nw.write(re.sub("\w*(?<![a-z])[0-9]|\.|\+|\-| ", '', line.strip()))
            else:
                nw.write("\n")
    process = subprocess.Popen(bashCommand.split(), stdout=subprocess.PIPE)
    output, error = process.communicate()


with open("oneline.pgn", "r") as infile, open("book.pgn", 'w') as outfile:
    for line in infile:
        if line.strip():
            outfile.write(line)


os.remove("oneline.pgn")
