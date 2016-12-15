#include <iostream>
#include <fstream>
#include <sstream>
#include <limits>
#include <vector>

#include "boost/program_options.hpp"

// Result
#include "AoC.hpp"
// puzzle solvers
#include "day00/Day00.hpp"
#include "day01/Day01.hpp"
#include "day02/Day02.hpp"
#include "day03/Day03.hpp"
#include "day04/Day04.hpp"
#include "day05/Day05.hpp"
#include "day06/Day06.hpp"
#include "day07/Day07.hpp"
#include "day08/Day08.hpp"
#include "day09/Day09.hpp"
#include "day10/Day10.hpp"
#include "day11/Day11.hpp"
#include "day12/Day12.hpp"
#include "day13/Day13.hpp"
#include "day14/Day14.hpp"
#include "day15/Day15.hpp"

using namespace std;
namespace po = boost::program_options;

// struct for command line options
struct Options {
    int day;
    int part;
    string inputset;
    bool verify;

    Options() : day(0), part(0), inputset(""), verify(false) {};
};

// contains general data about a puzzle, as well as a constructor from Options
class PuzzleData {
public:
    int day;
    int part;
    vector<string> input;

    PuzzleData(Options options) :
        day(options.day),
        part(options.part),
        input( read_input_set(get_path_name(), options.inputset) ),
        solution( read_solution(get_path_name(), options.inputset) ) {
    }

    ~PuzzleData(){};

    bool has_solution() {
        return solution != "";
    }

    bool verify_solution(const string check) {
        return solution == check;
    }

private:
    // keep the solution private and only provide access to it via the publicly
    // available verify_solution() method
    string solution;

    string get_path_name() {
        string day_string = to_string(day);
        while( day_string.length() < 2 ) {
            day_string = "0" + day_string;
        }

        return "../_inputs/day" + day_string + "/";
    }

    vector<string> read_input_set(const string pathname, const string inputset) {
        string fname = pathname + "/" + inputset + ".input";
        return read_file_contents(fname);
    }

    string read_solution(const string pathname, const string inputset) {
        string fname = pathname + "/" + inputset + ".solution";
        vector<string> tmp = read_file_contents(fname);

        string retval;
        if( tmp.size() > 0) {
            retval = tmp[0];
        }

        return retval;
    }

    vector<string> read_file_contents(const string fname) {
        vector<string> content;
        ifstream file(fname);
        string line;
        while(getline(file, line)) {
            content.push_back(line);
        }

        return content;
    }
};

Options get_options_interactive(const int argc, char** const argv);
//Result run_solver(const Options options);
Result run_solver(const PuzzleData config);
bool verify_solution(const string inputset, const string result);

int main(int argc, char *argv[]) {
    cout << "Advent of Code 2016 - C++ implementations" << endl;

    Options options = get_options_interactive(argc, argv);
    cout << "Day: " << options.day
         << " part: " << options.part
         << " input set: \"" << options.inputset
         << "\" verify: " << options.verify << endl;
    PuzzleData puzzle(options);
    Result res = run_solver(puzzle);

    if( res.success ) {
        cout << "Result:\n" << res.result << endl;

        if( options.verify ) {
            if( puzzle.has_solution() ) {
                bool check = puzzle.verify_solution(res.result);
                cout << "[VERIFY] This is "
                     << (check ? "" : " NOT ") << "correct!" << endl;
                return !check;
            }

            cout << "[VERIFY] No solution for this input available!" << endl;
        }

        return 0;
    } else {
        cout << "[ERROR] " << res.result << endl;
    }

    return 1;
}

Options get_options_interactive(const int argc, char** const argv) {
    Options options;

    po::options_description desc("Allowed options");
    desc.add_options()
    ("help", "Show this help message and exit")
    ("day", po::value<int>(&options.day),  "day")
    ("part", po::value<int>(&options.part), "part")
    ("inputset", po::value<string>(&options.inputset),
     "Name of the input set to use")
    ("verify", "Verify result with inputset.solution")
    ;

    po::variables_map vm;
    po::store(po::parse_command_line(argc, argv, desc), vm);
    po::notify(vm);

    if( vm.count("help") ) {
        cout << desc << "\n";
        exit(0);
    }

    if( !vm.count("day") ) {
        cout << "Please enter a day to run: ";
        while( !(cin >> options.day) ) {
            cout << "Please enter a valid number! " << flush;
            cin.clear();
            cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        };
    }

    if( !vm.count("part") ) {
        cout << "Please enter a part to run: ";
        while( !(cin >> options.part) ) {
            cout << "Please enter a valid number! " << flush;
            cin.clear();
            cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        };
    }

    if( !vm.count("inputset") ) {
        cout << "Please enter the name of the input set to use: ";
        cin >> options.inputset;
    }

    options.verify = (vm.count("verify") > 0);

    return options;
}

Result run_solver(const PuzzleData puzzle) {

    if( puzzle.input.size() < 1 ) {
        return {false, "No input data found! Either the specified input set "
                "doesn\'t exist or it is empty."};
    }

    switch(puzzle.day) {
    case 0: {
        Day00 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 1: {
        Day01 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 2: {
        Day02 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 3: {
        Day03 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 4: {
        Day04 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 5: {
        Day05 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 6: {
        Day06 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 7: {
        Day07 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 8: {
        Day08 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 9: {
        Day09 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 10: {
        Day10 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 11: {
        Day11 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 12: {
        Day12 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 13: {
        Day13 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 14: {
        Day14 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    case 15: {
        Day15 solver(puzzle.input);
        return solver.solve(puzzle.part);
    }
    default:
        return Result {false, "Solver for day "
            + to_string(puzzle.day) + " not found!"};
    }

    return Result {false, "Internal error!"};
}
