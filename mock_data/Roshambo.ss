contract Roshambo {
    state challengeIssued {
        owner: address, # p2
        checksum: u256,
        challenger: address
    }

    fun newChallengeIssued(checksum: u256, owner: address): Roshambo.challengeIssued {
        {
            owner,
            checksum,
            challenger: self.caller
        }
    }

    state challengeAccepted {
        owner: address, # p1
        checksum: u256,
        p2Selection: u8
    }

    function newChallengeAccepted(c: Roshambo.challengeIssued, selection: u8): Roshambo.challengeAccepted {
        {
            owner: c.challenger,
            checksum: c.checksum,
            p2_selection: selection,
            acceptor: self.caller
        }
    }

    state challengeFinalized {
        checksum: u256,
        p1: address,
        p1Selection: u8,
        p2: address
        p2Selection: u8,
        # 0: tie, 1: p1, 2: p2
        winner: u8 
    }

    fun checksum(selection: u8, salt: u32): str
        hash(hash(selection) + hash(salt))
    
    function newChallengeFinalized(c: Roshambo.challengeAccepted, selection: str, salt: str): Roshambo.challengeFinalized {
        assert(checksum(selection, salt) == c.checksum, "invalid p1 selection or salt")
        {
            checksum: c.checksum,
            p1: c.owner,
            p1Selection: selection,
            p2: c.acceptor,
            p2Selection: c.p2Selection,
            winner: (selection - c.p2Selection) % 3
        }
    }
}