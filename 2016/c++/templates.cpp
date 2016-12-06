// string.split() using boost
char_separator<char> sep(", ");
tokenizer<char_separator<char> > tokens(input[0], sep);
BOOST_FOREACH(string t, tokens) {
    char direction = t[0];
    int distance;

    stringstream( t.substr(1) ) >> distance;
    Direction dir {direction, distance};

    this->data.push_back(dir);
}
