-module(recover).
-export([recover/2]).

recover(OnError, Body) ->
    try Body() of
        Result -> Result
    catch
        _:Reason -> OnError(Reason)
    end.
