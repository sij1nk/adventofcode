main :-
	open('day01_transformed.txt', read, Stream),
	read(Stream, Lines),
	part1(Lines), part2(Lines), halt.

part1(Lines) :-
	member(X, Lines),
	member(Y, Lines),
	X+Y =:= 2020, !,
	Answer is X*Y,
	write(Answer), nl.

part2(Lines) :-
	member(X, Lines),
	member(Y, Lines),
	member(Z, Lines),
	X+Y+Z =:= 2020,
	!,
	Answer is X*Y*Z,
	write(Answer).
