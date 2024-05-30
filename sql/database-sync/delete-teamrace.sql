DELETE FROM teamrace WHERE map = $1 AND name = $2 AND time = $3 AND timestamp = $4 AND id = decode($5, 'hex')
