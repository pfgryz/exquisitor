from typing import Generic, TypeVar, List

Delegate = TypeVar("Delegate")


class Event(Generic[Delegate]):

    def __init__(self):
        self._delegates: List[Delegate] = []

    def add(self, delegate: Delegate) -> None:
        """
        Add a delegate to the list of delegates.

        Parameters
        ----------
        delegate : Delegate
            The delegate to add
        """
        if delegate not in self._delegates:
            self._delegates.append(delegate)

    def remove(self, delegate: Delegate) -> None:
        """
        Remove a delegate from the list of delegates.

        Parameters
        ----------
        delegate : Delegate
            The delegate to remove.
        """
        if delegate in self._delegates:
            self._delegates.remove(delegate)

    def clear(self) -> None:
        """
        Clear the list of delegates.
        """
        self._delegates.clear()

    def broadcast(self, *args, **kwargs) -> None:
        """
        Broadcast the events to the list of delegates.
        """
        for delegate in self._delegates:
            delegate(*args, **kwargs)
