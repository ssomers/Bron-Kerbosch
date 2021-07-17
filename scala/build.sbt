name := "Bron-Kerbosch"
version := "0.1"

scalaVersion := "2.13.6"
scalacOptions ++= Seq(
  "-unchecked",
  "-deprecation",
  "-feature",
  "-Xfatal-warnings"
  //"-Xdisable-assertions"
)
//noinspection Annotator,SpellCheckingInspection
libraryDependencies += "org.scalatest" %% "scalatest" % "3.2.5" % "test"
