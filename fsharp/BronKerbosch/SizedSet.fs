namespace BronKerbosch

open System.Collections

type SizedSet<'T when 'T: comparison> =
    { set: Set<'T>
      size: int }

    member this.IsEmpty: bool = this.set.IsEmpty

    member this.Contains(value: 'T) : bool = this.set.Contains(value)

    member this.Add(value: 'T) : SizedSet<'T> =
        assert not (this.set.Contains value)

        { set = this.set.Add(value)
          size = this.size + 1 }

    member this.Remove(value: 'T) : SizedSet<'T> =
        assert this.set.Contains value

        { set = this.set.Remove(value)
          size = this.size - 1 }

    interface Generic.IEnumerable<'T> with
        member this.GetEnumerator() : Generic.IEnumerator<'T> = (this.set :> seq<'T>).GetEnumerator()

    interface IEnumerable with
        member this.GetEnumerator() : IEnumerator =
            (this.set :> IEnumerable).GetEnumerator()
