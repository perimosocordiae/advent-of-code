use IO;

iter parseLines() {
  var lhs: string;
  var rhs: string;
  while readf("%/([%&]?[a-z]+)/ -> %/([a-z, ]+)/\n", lhs, rhs) {
    // TODO: parse first char of lhs: %, &, or nothing
    // TODO: parse rhs into list of strings
    yield (lhs, rhs);
  }
}

const Input = parseLines();
writeln(Input);
