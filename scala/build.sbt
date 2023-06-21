name := "Bron-Kerbosch"
version := "0.1"

scalaVersion := "2.13.8"
//noinspection Annotator,SpellCheckingInspection
libraryDependencies += "org.scalatest" %% "scalatest" % "3.2.15" % "test"
//noinspection SpellCheckingInspection
scalacOptions ++= Seq(
  "-unchecked",
  "-deprecation",
  "-feature",
  "-Xfatal-warnings"
)
scalacOptions += "-Xdisable-assertions"
