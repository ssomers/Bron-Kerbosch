name := "Bron-Kerbosch"
version := "0.1"

scalaVersion := "2.13.6"
//noinspection Annotator,SpellCheckingInspection
libraryDependencies += "org.scalatest" %% "scalatest" % "3.2.9" % "test"
//noinspection SpellCheckingInspection
scalacOptions ++= Seq(
  "-unchecked",
  "-deprecation",
  "-feature",
  "-Xfatal-warnings"
)
//scalacOptions += "-Xdisable-assertions"
