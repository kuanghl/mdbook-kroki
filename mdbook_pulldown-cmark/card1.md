````
```
Look! You can see my backticks.
```
````

```ruby
require 'redcarpet'
markdown = Redcarpet.new("Hello World!")
puts markdown.to_html
```

- [x] #739
- [ ] https://github.com/octo-org/octo-repo/issues/740
- [ ] Add delight to the experience when all tasks are complete :tada:

Table of Number Theory Functions
--------------------------------

The following table shows information about a few important functions
in number theory.

| Name                     | Notation       | First few values                      | Multiplicative property   |
| ------------------------ | -------------- | ------------------------------------- | ------------------------- |
| Möbius function          | $ \mu(n) $     | $ 1, -1, -1, 0, -1 $                  | Multiplicative            |
| Euler's totient function | $ \varphi(n) $ | $ 1, 1, 2, 2, 4 $                     | Multiplicative            |
| Mangoldt function        | $ \Lambda(n) $ | $ 0, \log 2, \log 3, \log 2, \log 5 $ | Not multiplicative        |
| Liouville's function     | $ \lambda(n) $ | $ 1, -1, -1, 1, -1 $                  | Completely multiplicative |