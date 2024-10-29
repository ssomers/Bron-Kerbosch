name := "Bron-Kerbosch"
version := "0.1"

scalaVersion := "3.5.2"
libraryDependencies += "org.scalatest" %% "scalatest" % "3.2.19" % "test"
scalacOptions ++= Seq(
  "-unchecked",
  "-deprecation",
  "-feature",
  "-Xfatal-warnings"
)
scalacOptions += "-Xelide-below 2147483647"
