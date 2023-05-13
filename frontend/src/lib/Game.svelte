<script lang="ts">
  import { Game, Move, Player, BoardCoordinate, Direction, PieceType } from './game';
  export let game = new Game();

  const after1move = game.attemptMove(new Move(
    Player.Beige,
    new BoardCoordinate(4, 5),
    Direction.S,
    Direction.N,
    [Direction.NE]
  ));
</script>

<div>
  {#if after1move}
    <table>
      {#each after1move.board as row}
        <tr>
          {#each row as col}
            <td>
              {#if col}
                {col.asString()}
              {:else}
                &nbsp;
              {/if}
            </td>
          {/each}
        </tr>
      {/each}
    </table>
  {:else}
    Illegal move
  {/if}
</div>
