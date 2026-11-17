package dev.rustchunk.minecraft1201;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonArray;
import com.google.gson.JsonObject;
import net.minecraft.SharedConstants;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.server.Bootstrap;
import net.minecraft.tags.BlockTags;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.levelgen.Heightmap;

import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.TreeMap;

/**
 * Exports Minecraft 1.20.1's own heightmap predicates and the primitive
 * state facts needed to independently verify them against the official
 * 1.20.1 block-tag resources. Compiled and run against checksum-pinned
 * Mojang-mapped game jars; never packaged in RustChunk.
 */
public final class Vanilla1201HeightmapFactsExporter {
    private Vanilla1201HeightmapFactsExporter() {}

    private static JsonObject exportFacts() {
        TreeMap<Integer, BlockState> ordered = new TreeMap<>();
        for (BlockState state : Block.BLOCK_STATE_REGISTRY) {
            int id = Block.getId(state);
            if (id < 0 || ordered.put(id, state) != null) {
                throw new IllegalStateException("Invalid or duplicate block-state ID: " + id);
            }
        }
        if (ordered.isEmpty()) {
            throw new IllegalStateException("Minecraft 1.20.1 block-state registry is empty");
        }

        JsonArray states = new JsonArray();
        for (int expectedId = 0; expectedId < ordered.size(); expectedId++) {
            BlockState state = ordered.get(expectedId);
            if (state == null) {
                throw new IllegalStateException("Block-state registry has a gap at ID " + expectedId);
            }
            String blockName = BuiltInRegistries.BLOCK.getKey(state.getBlock()).toString();
            boolean air = state.isAir();
            boolean blocksMotion = state.blocksMotion();
            boolean hasFluid = !state.getFluidState().isEmpty();
            boolean leavesTag = state.is(BlockTags.LEAVES);
            boolean motionBlocking =
                Heightmap.Types.MOTION_BLOCKING.isOpaque().test(state);
            boolean motionBlockingNoLeaves =
                Heightmap.Types.MOTION_BLOCKING_NO_LEAVES.isOpaque().test(state);

            JsonArray row = new JsonArray();
            row.add(expectedId);
            row.add(blockName);
            row.add(state.toString());
            row.add(air);
            row.add(blocksMotion);
            row.add(hasFluid);
            row.add(leavesTag);
            row.add(motionBlocking);
            row.add(motionBlockingNoLeaves);
            states.add(row);
        }

        JsonObject result = new JsonObject();
        result.addProperty("minecraft_version", "1.20.1");
        result.addProperty("schema", "rustchunk-heightmap-facts-v2");
        result.addProperty("predicate_source", "Heightmap.Types.MOTION_BLOCKING.isOpaque");
        result.addProperty(
            "no_leaves_predicate_source",
            "Heightmap.Types.MOTION_BLOCKING_NO_LEAVES.isOpaque"
        );
        result.addProperty("leaves_tag_probe", "BlockState.is(BlockTags.LEAVES)");
        result.add("states", states);
        return result;
    }

    public static void main(String[] args) throws Exception {
        if (args.length != 1) {
            throw new IllegalArgumentException(
                "Usage: Vanilla1201HeightmapFactsExporter <output.json>"
            );
        }
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();

        JsonObject facts = exportFacts();
        Path target = Path.of(args[0]).toAbsolutePath();
        Files.createDirectories(target.getParent());
        Path partial = target.resolveSibling(target.getFileName() + ".partial");
        Gson gson = new GsonBuilder().disableHtmlEscaping().create();
        Files.writeString(
            partial,
            gson.toJson(facts) + System.lineSeparator(),
            StandardCharsets.UTF_8
        );
        Files.move(partial, target, StandardCopyOption.REPLACE_EXISTING);
    }
}
