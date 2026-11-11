# Water Molecule Synchronization Solution

## Problem
Implement synchronization for hydrogen and oxygen threads to form water molecules. Each water molecule requires exactly 1 oxygen atom and 2 hydrogen atoms, and all threads in a molecule must complete together as a group before the next group starts.

## Solution Approach

The solution uses a **condition variable barrier** pattern with precise state tracking:

### Hydrogen Thread Flow
1. Acquire lock
2. **Barrier 1**: Wait until `h_count < 2 && o_count == 0` (space available for this group)
3. Increment `h_count`
4. If `h_count == 2`, notify oxygen that 2 hydrogens are ready
5. **Barrier 2**: Wait until `o_count == 1` (oxygen has joined this group)
6. Release lock and call `releaseHydrogen()`
7. Re-acquire lock, decrement `h_count`
8. If `h_count == 0` (last hydrogen in group), reset `o_count = 0` and notify oxygen

### Oxygen Thread Flow
1. Acquire lock
2. **Barrier 1**: Wait until `h_count == 2 && o_count == 0` (both hydrogens ready, no oxygen assigned)
3. Set `o_count = 1` to signal hydrogens
4. Notify all waiting hydrogens
5. Release lock and call `releaseOxygen()`
6. Re-acquire lock
7. **Barrier 2**: Wait until `o_count == 0` (hydrogens have reset it, meaning all completed)

## Key Design Insights

- **Dual barriers**: Hydrogens wait at 2 points (group assembly, then oxygen arrival); oxygen waits at 2 points (hydrogen readiness, then cleanup)
- **State reset in hydrogen**: Hydrogen is responsible for resetting `o_count` when all hydrogens finish, enabling the next oxygen to proceed
- **No spurious wakeups**: Every `notify_all()` changes actual state (`h_count`, `o_count`), preventing spurious condition check failures
- **Lock-release during critical sections**: Each thread releases the lock before calling `releaseXXX()`, preventing deadlocks

## Synchronization Guarantees

- ✓ **Exactly 2H + 1O per group**: Hydrogen only proceeds when `h_count < 2`, and oxygen only joins when `h_count == 2`
- ✓ **Group atomicity**: Next group cannot start until `h_count == 0` and `o_count == 0` (verified by conditions)
- ✓ **No deadlocks**: Locks are released before waiting; condition variables safely release locks while waiting
- ✓ **Progress**: When 2H arrive, oxygen is woken; when O joins, both H are woken; when H complete, O is woken for next group

## Implementation Details

```cpp
private:
    mutex lock;                    // Protects all shared state
    condition_variable cv;         // Signals state changes
    int h_count = 0;              // Active hydrogens [0..2]
    int o_count = 0;              // Oxygen assigned flag [0 or 1]
```

## Output Example

Input: 6 threads (H H O H H O)  
Possible outputs: `OHHOHH`, `OHHHHO`, `HOHHOH`, etc.  
All valid outputs have groups of 3 with exactly 1O + 2H each.

Example trace for "OHHOHH":
- H1 arrives: h_count=1, waits for h_count < 2 (satisfied)
- H2 arrives: h_count=2, notifies O
- O1 arrives: finds h_count==2 && o_count==0, sets o_count=1, notifies all
- H1,H2 wake: see o_count==1, exit barrier 2, call releaseH()
- Group 1 prints: "O HH" ✓
- H3,H4 arrive for group 2
- O2 sets up group 2
- Group 2 prints: "O HH" ✓

## Complexity

- **Time**: O(1) per thread (constant synchronization operations)
- **Space**: O(1) additional memory
- **Fairness**: FIFO ordering of molecule groups

