#include <iostream>
#include <fstream>
#include <sstream>
#include <limits>
#include <vector>

#include "day01/Day01.h"
#include "day02/Day02.h"
#include "day03/Day03.h"
#include "day04/Day04.h"

using namespace std;

bool get_day_part_input(const int argc, char** const argv, int& day, int& part,
        vector<string>& input);
bool get_day_part(const int argc, char** const argv, int& day, int& part);
bool get_input_for_day_part(int day, int part, vector<string>& input);
bool run_solver(int day, int part, vector<string> input, string& result);

int main(int argc, char *argv[]) {
    cout << "Advent of Code 2015 - C++ implementations" << endl;

    int day;
    int part;
    vector<string> input;
    bool check = get_day_part_input(argc, argv, day, part, input);
    if( !check ){
        cout << "Setting up the solver failed. Aborting" << endl;
        return 1;
    }

    string result;
    check = run_solver(day, part, input, result);
    if ( check ){
        cout << "Result:\n" << result << endl;
        return 0;
    } else {
        cout << "Something went wrong!" << endl;
    }

    return 1;
}

bool run_solver(int day, int part, vector<string> input, string& result){

    switch(day){
    case 1: {
        Day01 solver(input);
        return solver.solve(part, result);
    }
    case 2: {
        Day02 solver(input);
        return solver.solve(part, result);
    }
    case 3: {
        Day03 solver(input);
        return solver.solve(part, result);
    }
    case 4: {
        Day04 solver(input);
        return solver.solve(part, result);
    }
    }

    return false;
}

bool get_day_part_input(const int argc, char** const argv, int& day, int& part,
        vector<string>& input) {

    bool check = get_day_part(argc, argv, day, part);
    int counter = 0;
    while( !check && counter < 3 ){
        cout << "Invalid values specified! Please try again!" << endl;
        check = get_day_part(0, nullptr, day, part);
    }

    if( !check ){
        cout << "You failed to enter correct values " << counter <<
        " times in a row. Aborting!";
        return false;
    }

    check = get_input_for_day_part(day, part, input);
    return check;
}

// reads day and part from command line parameters or prompts the user if not
// provided
// returns true on success or false if any value could not correctly be parsed
bool get_day_part(const int argc, char** const argv, int& day, int& part){

    // reset variables
    day = part = 0;
    switch(argc){
    case 3: {
        // day & part were specified on command line
        stringstream input_day = stringstream(argv[1]);
        stringstream input_part = stringstream(argv[2]);

        input_day >> day;
        if ( !input_day ){
            cout << "Invalid value for day specified!" << endl;
        }

        input_part >> part;
        if ( !input_part ){
            cout << "Invalid value for part specified!" << endl;
        }

        if( input_day && input_part ){
            return true;
        }

        break;
    }
    case 2: {
        // only the day was specified
        stringstream input_day = stringstream(argv[1]);
        input_day >> day;
        if( !input_day ){
            cout << "Invalid value for day specified!" << endl;
            return false;
        }

        cout << "You have chosen day " << day << ". And which part? :";
        while( !(cin >> part) ){
            cout << "Please enter a valid number! " << flush;
            cin.clear();
            cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        };

        return true;
    }
    case 1: case 0: {
        // case 1: program was started without command line parameters
        // case 0: this function already returend false, so it was invoked
        // again with empty argc & argv, to force user prompts
        cout << "Which day do you want to solve? ";
        while( !(cin >> day) ){
            cout << "Please enter a valid number! " << flush;
            cin.clear();
            cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        };
        cout << "And which part? ";
        while( !(cin >> part) ){
            cout << "Please enter a valid number! " << flush;
            cin.clear();
            cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
        };

        return true;
    }
    default:
        // something bad happened
        cout << "Invalid parameters!" << endl;
        break;
    }

    return false;
}

bool get_input_for_day_part(int day, int part, vector<string>& input){
    string day_string = to_string(day);
    while( day_string.length() < 2 ){
        day_string = "0" + day_string;
    }
    string fname = "../inputs/" + day_string + ".txt";

    ifstream file(fname);
    string line;
    while(getline(file, line)){
        input.push_back(line);
    }

    return input.size() > 0;

}
