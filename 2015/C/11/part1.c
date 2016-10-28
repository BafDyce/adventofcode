// adventofcode - day 11
// part 1

#include <stdio.h>
#include <stdlib.h>

typedef struct String {
    char *s;
    int len;
} String;

String *import_data();
void increment_pw(String *pw);
int is_valid_pw(String *pw);
int contains_increasing_straight(String *pw);
int contains_two_pairs(String *pw);
void free_string(String *string);

int main(int argc, char *argv[]){
    int ii;

    printf("Advent of Code - day 11 | part 1\n");
    String *pw = import_data();
    printf("Current password is: %s\n", pw->s);

    do {
        increment_pw(pw);
    } while ( ! is_valid_pw(pw) );

    printf("Valid password found: %s\n", pw->s);

    free_string(pw);

    return 0;
}

void free_string(String *string) {
    free(string->s);
    free(string);
    string = NULL;
}

int is_valid_pw(String *pw){
    return contains_increasing_straight(pw) && contains_two_pairs(pw);
}

int contains_two_pairs(String *pw){
    int pair_found = 0;
    char *c, *cEnd, *cFirst;

    c = pw->s;
    cEnd = &(pw->s[pw->len-1]);

    // look for the first pair
    while ( c < cEnd ){
        // compare current character with next one
        if ( *c == *(c + 1) ){
            pair_found = 1;
            cFirst = c;
            break;
        }

        c++;
    }

    // only if we've found a pair so far, we'll look for another one
    if ( pair_found ){
        c += 2;
        while ( c < cEnd ){
            if ( *c == *(c + 1) && *c != *cFirst ){
                return 1;
            }

            c++;
        }
    }

    return 0;
}

int contains_increasing_straight(String *pw){
    int ii;

    for ( ii = 0; ii < pw->len - 2; ii++){
        if ( pw->s[ii] == pw->s[ii+1] - 1
                && pw->s[ii] == pw->s[ii + 2] - 2 ) {
            return 1;
        }
    }

    return 0;
}

void increment_pw(String *pw) {
    int ii;

    for( ii = pw->len - 1; ii >= 0; ii--){
        switch ( pw->s[ii] ) {
            case 'h':
            case 'j':
            case 'n':
                // skip bad letters
                pw->s[ii] += 2;
                return;
            case 'z':
                pw->s[ii] = 'a';
                break;
            default:
                pw->s[ii] += 1;
                return;
        }
    }
}

String *import_data() {
    FILE *file;
    int fsize;
    String *data;

    data = malloc( sizeof(String));
    if ( data == NULL ){
        fprintf(stderr, "Failed to allocate memory!\n");
        exit(1);
    }

    file = fopen("../../inputs/11.txt", "r");
    if ( file == NULL ){
        fprintf(stderr, "Could not open file!\n");
        exit(1);
    }

    fseek(file, 0, SEEK_END);
    fsize = ftell(file);

    data->s = malloc( sizeof(char) * fsize );
    if ( data->s == NULL ){
        fprintf(stderr, "Failed to allocate memory!\n");
        exit(1);
    }

    rewind(file);
    fgets(data->s, fsize, file);
    data->len = fsize - 1;

    fclose(file);
    return data;
}
