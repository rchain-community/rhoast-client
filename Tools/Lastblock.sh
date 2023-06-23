##Check all validator last block
i=0
while [ $i -lt 30 ]; do
        lastblock=$(curl -s https://node25.root-shard.mainnet.rchain.coop/api/blocks | jq ".[0].blockNumber")
        echo node$i $lastblock
        i=$(($i+1))
#exit
done
