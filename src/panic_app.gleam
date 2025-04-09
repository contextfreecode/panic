import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string

pub fn main() {
  // https://tour.gleam.run/advanced-features/externals/
  list.range(1, 3)
  |> list.each(fn(text) {
    text
    |> process_text()
    |> io.println()
  })
}

fn process_text(id: Int) -> String {
  retrieve_text(id)
  // Can't use lazy_unwrap because it doesn't get the error value.
  |> result.map_error(fn(err) { panic as string.inspect(err) })
  |> result.unwrap("never fails")
  |> string.to_utf_codepoints()
  |> rotate_back()
  |> string.from_utf_codepoints()
}

const texts = ["smile", "tears", "🐻‍❄️"]

fn retrieve_text(id: Int) -> Result(String, Error) {
  case Nil {
    _ if id <= 0 -> Error(Nil)
    _ ->
      texts
      |> list.drop(id - 1)
      |> list.first()
  }
  |> result.map_error(fn(_) {
    NotFoundError("404 - Not found: " <> int.to_string(id))
  })
}

fn rotate_back(codes: List(a)) -> List(a) {
  [codes |> list.drop(1), codes |> list.take(1)]
  |> list.flatten()
}

type Error {
  NotFoundError(message: String)
}
