
#creating a varible that is the name of the compiler being used
CC=rustc

#using all the run the rule named rule (can now just run the command make without specifyig a file)
all: rule
#create a rule that depends on a specific file
rule: main.rs
	$(CC) main.rs 
#the command used to compile the file

#creating a clean rule to delete all generated files
clean:
	rm main