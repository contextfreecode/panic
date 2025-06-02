import std/core/exn

pub fun main()
  run()
  println("Still alive.")

pub fun run(): console ()
  with final ctl throw-exn(exn)
    println("exn: " ++ exn.show)
  for(1, 3) fn(id)
    val result = process-text(id)
    println(result)

pub fun my-for(start: int, end: int, action: (int) -> ()): ()
  for(start, end, action)

fun process-text(id: int): exn string
  retrieve-text(id).untry.split("").reverse-join

fun retrieve-text(id: int): error<string>
  try fn() texts[id - 1]

val texts = vector(["tar", "flow"])
