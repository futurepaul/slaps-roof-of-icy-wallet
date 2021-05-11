# "Slaps roof of wallet" 

A toy Bitcoin wallet built with BDK and Iced. There's also a [Druid version](https://github.com/futurepaul/slaps-roof-of-wallet). Should eventually look like this:

![slaps-roof-figma](https://user-images.githubusercontent.com/543668/117876483-6e821700-b271-11eb-9751-5cdce71487b9.png)

Right now it looks like this:

![iced-screenshots](https://user-images.githubusercontent.com/543668/117876470-6b872680-b271-11eb-9900-ad1b49b8bdc7.png)

For testing I use a `coldcard-export.json` from a burner coldcard:

```
Advanced > MicroSD Card > Export Wallet > Generic JSON
```

And my local Bitcoin Regtest Electrs server is [nigiri](https://github.com/vulpemventures/nigiri).

## TODO

- [x] multiple screens
- [x] nav between multiples screens
- [x] file dialog
- [x] import coldcard json
- [x] display some coldcard info
- [ ] create bdk wallet
- [ ] spend money
