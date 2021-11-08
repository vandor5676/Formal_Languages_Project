

def startState(char):
    if(char == 'a'):
        return 1
    else:
        return 0

def q1(char):
    if(char == 'a'):
        return 1
    elif(char == 'b'):
        return 2
    else:
        return 0

def q2(char):
    if(char=='c'):
        return 3
    elif(char=='a'):
        return 1
    else:
        return 0

def acceptState():
    return 3


def isAccepted(input = "aababcaaa"):
    length = len(input)
    dfa  =0
    for i in range(length):
        if(dfa == 0):
            dfa= startState(input[i])
        elif (dfa==1):
             dfa= q1(input[i])
        elif (dfa==2):
             dfa= q2(input[i])
        elif (dfa==3):
             dfa= acceptState()
    if(dfa == 3):
        print("The string was accepted") 
    else:
        print("The string was Not accepted")                            


if __name__ == "__main__":
    print("DFA in Python 🐍 ");
    isAccepted()