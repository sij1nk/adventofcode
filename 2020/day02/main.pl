:- use_module(library(lists), [findall/3, length/2, nth1/3]).

main :-
	open('input_transformed.txt', read, Stream),
	read(Stream, Pws),
	part1(Pws), part2(Pws), halt.

part1(Pws) :-
	findall(Pw, (member(Pw, Pws), validate1(Pw)), Valid_Pws),
	length(Valid_Pws, Answer),
	write(Answer), nl.

validate1([Min-Max, Char, String]) :-
	atom_codes(String, Codes),
	atom_codes(Char, Code),
	findall(MatchingCode, (member(MatchingCode, Codes), MatchingCode =:= Code), MatchingCodes),
	length(MatchingCodes, MatchingCodesLength),
	MatchingCodesLength =< Max,
	MatchingCodesLength >= Min.

part2(Pws) :-
	findall(Pw, (member(Pw, Pws), validate2(Pw)), Valid_Pws),
	length(Valid_Pws, Answer),
	write(Answer).

validate2([I1-I2, Char, String]) :-
	atom_codes(String, Codes),
	atom_codes(Char, Code),
	nth1(I1, Codes, C1),
	nth1(I2, Codes, C2),
	\+ (C1 =:= Code, C2 =:= Code),
	(C1 =:= Code; C2 =:= Code).
