module Main exposing (..)

import Browser
import Html exposing (Html, h1, text)
import InteropDefinitions
import InteropPorts


type alias Model =
    { name : String }


greet : Cmd msg
greet =
    "Hello from elm-ts-interop!"
        |> InteropDefinitions.Alert
        |> InteropPorts.fromElm


view : Model -> Html ()
view model =
    h1 [] [ text ("Hello " ++ model.name ++ " World!!!") ]


main : Program () Model ()
main =
    Browser.element
        { init = \_ -> ( { name = "Elm" }, greet )
        , view = view
        , update = \_ model -> ( model, Cmd.none )
        , subscriptions = \_ -> Sub.none
        }
