import sys
import solver

if __name__ == "__main__":
    address: str | None = None
    maze: str | None = None

    print("maze-solver (Python)")

    while (i := 0) < len(sys.argv) - 1:
        match sys.argv[i]:
            case "--url":
                address = sys.argv[i + 1]
            case "--maze":
                maze = sys.argv[i + 1]
            case "--help":
                print("Usage: solver.py [OPTION] [ARG]\n")
                print(" [OPTION]  [ARG]")
                print("   --url   Specify a custom URL for the API Calls.")
                print("           Default URL: https://gtm.delary.dev")
                print("   --maze  Specify a custom maze for the program.")
                print('           Default maze: "maze-sample"')
                print("   --help  Displays this help text.")
                quit()

        i += 1

    solver.solver(address, maze)
