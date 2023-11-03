import sys
import solver

if __name__ == "__main__":
    address: str | None = None
    maze: str | None = None

    print("maze-solver (Python)")

    i = 0
    while i < len(sys.argv):
        match sys.argv[i]:
            case "--url":
                address = sys.argv[i + 1]
            case "--maze":
                maze = sys.argv[i + 1]
            case "--help":
                print("Usage: main.py [OPTION] [ARG]\n")
                print(" [OPTION]  [OPTION ARG]")
                print(" --url     Specify a custom URL for the API Calls.")
                print("           Default URL: https://gtm.delary.dev")
                print("")
                print(" --maze    Specify a custom maze for the program.")
                print('           Default maze: "maze-sample"')
                print("")
                print(" --help    Displays this help text. (No args)")
                print("")
                quit()

        i += 1

    solver.solver(address, maze)
