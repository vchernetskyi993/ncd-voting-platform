module Main exposing (..)

import Browser
import Html exposing (Html, h1, text)
import InteropDefinitions exposing (Flags)
import InteropPorts


type alias Model =
    { accountId : String }


greet : Cmd msg
greet =
    "Hello from elm-ts-interop!"
        |> InteropDefinitions.Alert
        |> InteropPorts.fromElm


init : Flags -> ( Model, Cmd msg )
init flags =
    ( { accountId = flags.accountId }, greet )


view : Model -> Html ()
view model =
    h1 [] [ text ("Hello " ++ model.accountId ++ "!!!") ]


main : Program Flags Model ()
main =
    Browser.element
        { init = init
        , view = view
        , update = \_ model -> ( model, Cmd.none )
        , subscriptions = \_ -> Sub.none
        }
