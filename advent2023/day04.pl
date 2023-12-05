use v6;

my $part1 = 0;
my @card_copies;
for "inputs/04.full".IO.lines {
    # Strip off the "Card " prefix.
    my ($card_num, $line) = .substr(5).split(': ');
    $card_num = $card_num.Int - 1;
    # Split the line into two parts by |
    my ($winners, $numbers) = $line.split(' | ');
    # Convert each part into a sorted list of numbers.
    my @winners = $winners.split(rx/\s+/).map(*.Int).sort;
    my @numbers = $numbers.split(rx/\s+/).map(*.Int).sort;
    # Find the intersection size of the two lists.
    my $num_matched = (@winners ∩ @numbers).elems;

    # Convert the intersection size into a point value: 2^(n-1) if n > 0
    my $points = $num_matched > 0 ?? 2 ** ($num_matched - 1) !! 0;
    $part1 += $points;

    # Keep track of the card copies.
    @card_copies[$card_num] += 1;
    # Update any copied cards after card_num.
    for 1 .. $num_matched {
        @card_copies[$card_num + $_] += @card_copies[$card_num];
    }
}

say "Part 1: ", $part1;\
say "Part 2: ", [+] @card_copies;
