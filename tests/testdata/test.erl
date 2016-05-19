%% $ erlc +debug_info test.erl
-module(test).

-export([hello/1]).

-spec hello(term()) -> ok.
hello(Name) ->
    io:format("Hello ~p!", [Name]),
    ok.
