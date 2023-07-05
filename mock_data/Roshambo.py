"""
Roshambo
"""


def simple_hash(input):
    # Very simple, non-cryptographic hash function
    return sum(ord(c) for c in str(input)) % 256


def new_challenge_issued(checksum, owner):
    return {
        'owner': owner,
        'checksum': checksum,
        'challenger': 'self.caller',  # replace with actual value
    }


def new_challenge_accepted(c, selection):
    return {
        'owner': c['challenger'],
        'checksum': c['checksum'],
        'p2_selection': selection,
        'acceptor': 'self.caller',  # replace with actual value
    }


def checksum(selection, salt):
    # This might not be the same hash function as in the original code.
    selection_hash = simple_hash(selection)
    salt_hash = simple_hash(salt)
    return simple_hash(selection_hash + salt_hash)


def new_challenge_finalized(c, selection, salt):
    assert checksum(
        selection, salt) == c['checksum'], "invalid p1 selection or salt"
    return {
        'checksum': c['checksum'],
        'p1': c['owner'],
        'p1Selection': selection,
        'p2': c['acceptor'],
        'p2Selection': c['p2_selection'],
        'winner': (selection - c['p2_selection']) % 3
    }
